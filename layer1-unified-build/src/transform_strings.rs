use crate::transform_safe::safe_string_transform;

/// Remove environment variable references (safe string transformation)
pub fn fix_env_vars(content: &str) -> String {
    safe_string_transform(content, "FIX_ENV_VARS", "Fix environment variable references", |content: &str| {
        content.replace("env ! (\"CFG_RELEASE_CHANNEL\")", "\"dev\"")
    })
}

/// Fix attribute spacing (safe string transformation)  
pub fn fix_attribute_spacing(content: &str) -> String {
    safe_string_transform(content, "FIX_ATTR_SPACING", "Fix attribute spacing", |content: &str| {
        content.replace("# [", "#[")
    })
}

/// Remove problematic crate attributes (safe string transformation)
pub fn remove_crate_attrs(content: &str) -> String {
    safe_string_transform(content, "REMOVE_CRATE_ATTRS", "Remove problematic crate attributes", |content: &str| {
        content.replace("# [allow (internal_features)] # [allow (rustc :: untranslatable_diagnostic)]", "")
    })
}
