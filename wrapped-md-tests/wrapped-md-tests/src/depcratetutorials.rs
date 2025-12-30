// Generated macro for tutorials (module)
macro_rules! Depcratetutorials {
() => {
// Module: crate
// Provides: {"tutorials"}
// Dependencies: {}
mod tutorials { # [doc = include_str ! ("../../../tutorials/quickstart.md")] mod quickstart_md { } # [doc = include_str ! ("../../../tutorials/date-picker.md")] mod date_picker_md { } # [doc = include_str ! ("../../../tutorials/data-provider-runtime.md")] mod data_provider_runtime_md { } # [doc = include_str ! ("../../../tutorials/data-management.md")] mod data_management_md { } }
};
}
