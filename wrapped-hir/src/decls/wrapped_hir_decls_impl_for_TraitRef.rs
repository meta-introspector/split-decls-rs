use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<'db> TraitRef<'db> {
    pub(crate) fn new_with_resolver(
        db: &'db dyn HirDatabase,
        resolver: &Resolver<'_>,
        trait_ref: hir_ty::next_solver::TraitRef<'db>,
    ) -> Self {
        let env = resolver
            .generic_def()
            .map_or_else(
                || TraitEnvironment::empty(resolver.krate()),
                |d| db.trait_environment(d),
            );
        TraitRef { env, trait_ref }
    }
    pub fn trait_(&self) -> Trait {
        Trait {
            id: self.trait_ref.def_id.0,
        }
    }
    pub fn self_ty(&self) -> TypeNs<'_> {
        let ty = self.trait_ref.self_ty();
        TypeNs {
            env: self.env.clone(),
            ty,
        }
    }
    /// Returns `idx`-th argument of this trait reference if it is a type argument. Note that the
    /// first argument is the `Self` type.
    pub fn get_type_argument(&self, idx: usize) -> Option<TypeNs<'db>> {
        self.trait_ref
            .args
            .as_slice()
            .get(idx)
            .and_then(|arg| arg.ty())
            .map(|ty| TypeNs {
                env: self.env.clone(),
                ty,
            })
    }
}
