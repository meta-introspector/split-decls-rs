use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn apply_trait_bounds(
    mut generics: Generics,
    lifetime: LifetimeParam,
    container_attrs: &ContainerAttributes,
) -> Result<Generics> {
    if let Some(config_bounds) = &container_attrs.bounds {
        let mut config_bounds_applied = 0;
        for param in generics.params.iter_mut() {
            if let GenericParam::Type(type_param) = param {
                if let Some(replacement) = config_bounds
                    .iter()
                    .flatten()
                    .find(|p| p.ident == type_param.ident)
                {
                    *type_param = replacement.clone();
                    config_bounds_applied += 1;
                } else {
                    type_param.bounds = Default::default();
                    type_param.default = None;
                }
            }
        }
        let config_bounds_supplied = config_bounds
            .iter()
            .map(|bounds| bounds.len())
            .sum::<usize>();
        if config_bounds_applied != config_bounds_supplied {
            return Err(Error::new(
                Span::call_site(),
                format!(
                    "invalid `{}` attribute. too many bounds, only {} out of {} are applicable",
                    ARBITRARY_ATTRIBUTE_NAME, config_bounds_applied, config_bounds_supplied,
                ),
            ));
        }
        Ok(generics)
    } else {
        Ok(add_trait_bounds(generics, lifetime))
    }
}
