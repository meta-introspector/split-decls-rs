macro_rules! RequestedLevel {
    () => {
        # [derive (Subdiagnostic)] # [note (lint_requested_level)] pub (crate) struct RequestedLevel < 'a > { pub level : Level , pub lint_name : & 'a str , }
    };
}

RequestedLevel!();