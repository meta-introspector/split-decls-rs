use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[macro_export]
macro_rules! mkbuildrs {
    () => {
        define_package_config!()
    };
}
