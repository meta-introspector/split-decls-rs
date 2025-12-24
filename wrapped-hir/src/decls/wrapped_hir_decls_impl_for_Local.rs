use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Local {
    pub fn is_param(self, db: &dyn HirDatabase) -> bool {
        let src = self.primary_source(db);
        match src.source.value {
            Either::Left(pat) => {
                pat.syntax()
                    .ancestors()
                    .map(|it| it.kind())
                    .take_while(|&kind| {
                        ast::Pat::can_cast(kind) || ast::Param::can_cast(kind)
                    })
                    .any(ast::Param::can_cast)
            }
            Either::Right(_) => true,
        }
    }
    pub fn as_self_param(self, db: &dyn HirDatabase) -> Option<SelfParam> {
        match self.parent {
            DefWithBodyId::FunctionId(func) if self.is_self(db) => {
                Some(SelfParam { func })
            }
            _ => None,
        }
    }
    pub fn name(self, db: &dyn HirDatabase) -> Name {
        let body = db.body(self.parent);
        body[self.binding_id].name.clone()
    }
    pub fn is_self(self, db: &dyn HirDatabase) -> bool {
        self.name(db) == sym::self_
    }
    pub fn is_mut(self, db: &dyn HirDatabase) -> bool {
        let body = db.body(self.parent);
        body[self.binding_id].mode == BindingAnnotation::Mutable
    }
    pub fn is_ref(self, db: &dyn HirDatabase) -> bool {
        let body = db.body(self.parent);
        matches!(
            body[self.binding_id].mode, BindingAnnotation::Ref |
            BindingAnnotation::RefMut
        )
    }
    pub fn parent(self, _db: &dyn HirDatabase) -> DefWithBody {
        self.parent.into()
    }
    pub fn module(self, db: &dyn HirDatabase) -> Module {
        self.parent(db).module(db)
    }
    pub fn ty(self, db: &dyn HirDatabase) -> Type<'_> {
        let def = self.parent;
        let infer = db.infer(def);
        let ty = infer[self.binding_id];
        Type::new(db, def, ty)
    }
    /// All definitions for this local. Example: `let (a$0, _) | (_, a$0) = it;`
    pub fn sources(self, db: &dyn HirDatabase) -> Vec<LocalSource> {
        let (body, source_map) = db.body_with_source_map(self.parent);
        match body.self_param.zip(source_map.self_param_syntax()) {
            Some((param, source)) if param == self.binding_id => {
                let root = source.file_syntax(db);
                vec![
                    LocalSource { local : self, source : source.map(| ast |
                    Either::Right(ast.to_node(& root))), }
                ]
            }
            _ => {
                source_map
                    .patterns_for_binding(self.binding_id)
                    .iter()
                    .map(|&definition| {
                        let src = source_map.pat_syntax(definition).unwrap();
                        let root = src.file_syntax(db);
                        LocalSource {
                            local: self,
                            source: src
                                .map(|ast| match ast.to_node(&root) {
                                    Either::Right(ast::Pat::IdentPat(it)) => Either::Left(it),
                                    _ => unreachable!("local with non ident-pattern"),
                                }),
                        }
                    })
                    .collect()
            }
        }
    }
    /// The leftmost definition for this local. Example: `let (a$0, _) | (_, a) = it;`
    pub fn primary_source(self, db: &dyn HirDatabase) -> LocalSource {
        let (body, source_map) = db.body_with_source_map(self.parent);
        match body.self_param.zip(source_map.self_param_syntax()) {
            Some((param, source)) if param == self.binding_id => {
                let root = source.file_syntax(db);
                LocalSource {
                    local: self,
                    source: source.map(|ast| Either::Right(ast.to_node(&root))),
                }
            }
            _ => {
                source_map
                    .patterns_for_binding(self.binding_id)
                    .first()
                    .map(|&definition| {
                        let src = source_map.pat_syntax(definition).unwrap();
                        let root = src.file_syntax(db);
                        LocalSource {
                            local: self,
                            source: src
                                .map(|ast| match ast.to_node(&root) {
                                    Either::Right(ast::Pat::IdentPat(it)) => Either::Left(it),
                                    _ => unreachable!("local with non ident-pattern"),
                                }),
                        }
                    })
                    .unwrap()
            }
        }
    }
}
