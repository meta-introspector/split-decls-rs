// Generated macro for tests (module)
macro_rules! Depcrate_descriptiontests {
() => {
// Module: crate::description
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: Description ; use crate :: prelude :: * ; use crate :: Result ; use indoc :: indoc ; # [test] fn renders_single_fragment () -> Result < () > { let description : Description = "A B C" . into () ; verify_that ! (description , displays_as (eq ("A B C"))) } # [test] fn renders_two_fragments () -> Result < () > { let description = ["A B C" . to_string () , "D E F" . to_string ()] . into_iter () . collect :: < Description > () ; verify_that ! (description , displays_as (eq ("A B C\nD E F"))) } # [test] fn nested_description_is_indented () -> Result < () > { let description = Description :: new () . text ("Header") . nested (["A B C" . to_string ()] . into_iter () . collect :: < Description > ()) ; verify_that ! (description , displays_as (eq ("Header\n  A B C"))) } # [test] fn nested_description_indents_two_elements () -> Result < () > { let description = Description :: new () . text ("Header") . nested (["A B C" . to_string () , "D E F" . to_string ()] . into_iter () . collect :: < Description > () ,) ; verify_that ! (description , displays_as (eq ("Header\n  A B C\n  D E F"))) } # [test] fn nested_description_indents_one_element_on_two_lines () -> Result < () > { let description = Description :: new () . text ("Header") . nested ("A B C\nD E F" . into ()) ; verify_that ! (description , displays_as (eq ("Header\n  A B C\n  D E F"))) } # [test] fn single_fragment_renders_with_bullet_when_bullet_list_enabled () -> Result < () > { let description = Description :: new () . text ("A B C") . bullet_list () ; verify_that ! (description , displays_as (eq ("* A B C"))) } # [test] fn single_nested_fragment_renders_with_bullet_when_bullet_list_enabled () -> Result < () > { let description = Description :: new () . nested ("A B C" . into ()) . bullet_list () ; verify_that ! (description , displays_as (eq ("* A B C"))) } # [test] fn two_fragments_render_with_bullet_when_bullet_list_enabled () -> Result < () > { let description = Description :: new () . text ("A B C") . text ("D E F") . bullet_list () ; verify_that ! (description , displays_as (eq ("* A B C\n* D E F"))) } # [test] fn two_nested_fragments_render_with_bullet_when_bullet_list_enabled () -> Result < () > { let description = Description :: new () . nested ("A B C" . into ()) . nested ("D E F" . into ()) . bullet_list () ; verify_that ! (description , displays_as (eq ("* A B C\n* D E F"))) } # [test] fn single_fragment_with_more_than_one_line_renders_with_one_bullet () -> Result < () > { let description = Description :: new () . text ("A B C\nD E F") . bullet_list () ; verify_that ! (description , displays_as (eq ("* A B C\n  D E F"))) } # [test] fn single_fragment_renders_with_enumeration_when_enumerate_enabled () -> Result < () > { let description = Description :: new () . text ("A B C") . enumerate () ; verify_that ! (description , displays_as (eq ("0. A B C"))) } # [test] fn two_fragments_render_with_enumeration_when_enumerate_enabled () -> Result < () > { let description = Description :: new () . text ("A B C") . text ("D E F") . enumerate () ; verify_that ! (description , displays_as (eq ("0. A B C\n1. D E F"))) } # [test] fn single_fragment_with_two_lines_renders_with_one_enumeration_label () -> Result < () > { let description = Description :: new () . text ("A B C\nD E F") . enumerate () ; verify_that ! (description , displays_as (eq ("0. A B C\n   D E F"))) } # [test] fn multi_digit_enumeration_renders_with_correct_offset () -> Result < () > { let description = ["A B C\nD E F" ; 11] . into_iter () . map (str :: to_string) . collect :: < Description > () . enumerate () ; verify_that ! (description , displays_as (eq (indoc ! ("
                 0. A B C
                    D E F
                 1. A B C
                    D E F
                 2. A B C
                    D E F
                 3. A B C
                    D E F
                 4. A B C
                    D E F
                 5. A B C
                    D E F
                 6. A B C
                    D E F
                 7. A B C
                    D E F
                 8. A B C
                    D E F
                 9. A B C
                    D E F
                10. A B C
                    D E F")))) } # [test] fn new_is_empty () -> Result < () > { verify_that ! (Description :: new () , predicate (Description :: is_empty)) } # [test] fn text_is_not_empty () -> Result < () > { verify_that ! (Description :: new () . text ("something") , not (predicate (Description :: is_empty))) } # [test] fn new_zero_length () -> Result < () > { verify_that ! (Description :: new () . len () , eq (0)) } # [test] fn text_one_length () -> Result < () > { verify_that ! (Description :: new () . text ("something") . len () , eq (1)) } }
};
}
