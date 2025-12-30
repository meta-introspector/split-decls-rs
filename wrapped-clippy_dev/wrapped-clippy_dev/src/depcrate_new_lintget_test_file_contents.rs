// Generated macro for get_test_file_contents (function)
macro_rules! Depcrate_new_lintget_test_file_contents {
() => {
// Module: crate::new_lint
// Provides: {"get_test_file_contents"}
// Dependencies: {}
fn get_test_file_contents (lint_name : & str , msrv : bool) -> String { let mut test = formatdoc ! (r"
        #![warn(clippy::{lint_name})]

        fn main() {{
            // test code goes here
        }}
    ") ; if msrv { let _ = writedoc ! (test , r#"

                // TODO: set xx to the version one below the MSRV used by the lint, and yy to
                // the version used by the lint
                #[clippy::msrv = "1.xx"]
                fn msrv_1_xx() {{
                    // a simple example that would trigger the lint if the MSRV were met
                }}

                #[clippy::msrv = "1.yy"]
                fn msrv_1_yy() {{
                    // the same example as above
                }}
            "#) ; } test }
};
}
