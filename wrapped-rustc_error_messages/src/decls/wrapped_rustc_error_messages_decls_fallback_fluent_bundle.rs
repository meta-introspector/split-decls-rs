use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Return the default `FluentBundle` with standard "en-US" diagnostic messages.
#[instrument(level = "trace", skip(resources))]
pub fn fallback_fluent_bundle(
    resources: Vec<&'static str>,
    with_directionality_markers: bool,
) -> LazyFallbackBundle {
    Arc::new(
        LazyLock::new(
            Box::new(move || {
                let mut fallback_bundle = new_bundle(vec![langid!("en-US")]);
                register_functions(&mut fallback_bundle);
                fallback_bundle.set_use_isolating(with_directionality_markers);
                for resource in resources {
                    let resource = FluentResource::try_new(resource.to_string())
                        .expect("failed to parse fallback fluent resource");
                    fallback_bundle.add_resource_overriding(resource);
                }
                fallback_bundle
            }),
        ),
    )
}
