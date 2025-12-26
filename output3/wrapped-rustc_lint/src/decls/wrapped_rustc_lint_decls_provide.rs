use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub fn provide(providers: &mut Providers) {
    levels::provide(providers);
    expect::provide(providers);
    foreign_modules::provide(providers);
    *providers = Providers {
        lint_mod,
        ..*providers
    };
}
