use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<'db> GenericSubstitution<'db> {
    fn new(def: GenericDefId, subst: GenericArgs<'db>, env: Arc<TraitEnvironment<'db>>) -> Self {
        Self { def, subst, env }
    }
    pub fn types(&self, db: &'db dyn HirDatabase) -> Vec<(Symbol, Type<'db>)> {
        let container = match self.def {
            GenericDefId::ConstId(id) => Some(id.lookup(db).container),
            GenericDefId::FunctionId(id) => Some(id.lookup(db).container),
            GenericDefId::TypeAliasId(id) => Some(id.lookup(db).container),
            _ => None,
        };
        let container_type_params = container
            .and_then(|container| match container {
                ItemContainerId::ImplId(container) => Some(container.into()),
                ItemContainerId::TraitId(container) => Some(container.into()),
                _ => None,
            })
            .map(|container| {
                db.generic_params(container)
                    .iter_type_or_consts()
                    .filter_map(|param| match param.1 {
                        TypeOrConstParamData::TypeParamData(param) => Some(param.name.clone()),
                        TypeOrConstParamData::ConstParamData(_) => None,
                    })
                    .collect::<Vec<_>>()
            });
        let generics = db.generic_params(self.def);
        let type_params = generics
            .iter_type_or_consts()
            .filter_map(|param| match param.1 {
                TypeOrConstParamData::TypeParamData(param) => Some(param.name.clone()),
                TypeOrConstParamData::ConstParamData(_) => None,
            });
        let parent_len = self.subst.len()
            - generics
                .iter_type_or_consts()
                .filter(|g| matches!(g.1, TypeOrConstParamData::TypeParamData(..)))
                .count();
        let container_params = self.subst.as_slice()[..parent_len]
            .iter()
            .filter_map(|param| param.ty())
            .zip(container_type_params.into_iter().flatten());
        let self_params = self.subst.as_slice()[parent_len..]
            .iter()
            .filter_map(|param| param.ty())
            .zip(type_params);
        container_params
            .chain(self_params)
            .filter_map(|(ty, name)| {
                Some((
                    name?.symbol().clone(),
                    Type {
                        ty,
                        env: self.env.clone(),
                    },
                ))
            })
            .collect()
    }
}
