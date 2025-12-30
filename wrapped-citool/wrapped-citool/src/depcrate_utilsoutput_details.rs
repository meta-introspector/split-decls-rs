// Generated macro for output_details (function)
macro_rules! Depcrate_utilsoutput_details {
() => {
// Module: crate::utils
// Provides: {"output_details"}
// Dependencies: {}
# [doc = " Outputs a HTML <details> section with the provided summary."] # [doc = " Output printed by `func` will be contained within the section."] pub fn output_details < F > (summary : & str , func : F) where F : FnOnce () , { println ! (r"<details>
<summary>{summary}</summary>
") ; func () ; println ! ("</details>\n") ; }
};
}
