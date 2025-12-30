// Generated macro for make_summary_table (function)
macro_rules! Depcrate_test_linksmake_summary_table {
() => {
// Module: crate::test_links
// Provides: {"make_summary_table"}
// Dependencies: {}
# [doc = " Generates an HTML table summarizing the coverage of the testsuite."] pub fn make_summary_table (book : & Book , tests : & RuleToTests , rules : & Rules) -> String { let ch_to_rules = invert_rule_map (rules) ; let mut table = String :: from (TABLE_START) ; let mut total_rules = 0 ; let mut total_tests = 0 ; let mut total_uncovered = 0 ; for (item_index , item) in book . iter () . enumerate () { let BookItem :: Chapter (ch) = item else { continue ; } ; let Some (ch_path) = & ch . path else { continue ; } ; let level = ch . number . as_ref () . map (| ch | ch . len () - 1) . unwrap_or_default () as u32 ; let html_path = ch_path . with_extension ("html") . to_str () . unwrap () . replace ('\\' , "/") ; let number = ch . number . as_ref () . map (| n | n . to_string ()) . unwrap_or_default () ; let mut num_rules = 0 ; let mut num_tests_str = String :: from ("") ; let mut uncovered_str = String :: from ("") ; let mut coverage_str = String :: from ("") ; if let Some (rules) = ch_to_rules . get (ch_path) { num_rules = rules . len () ; total_rules += num_rules ; let num_tests = rules . iter () . map (| rule | tests . get (rule) . map (| ts | ts . len ()) . unwrap_or_default ()) . sum :: < usize > () ; total_tests += num_tests ; num_tests_str = num_tests . to_string () ; let uncovered_rules : Vec < _ > = rules . iter () . filter (| rule | ! tests . contains_key (rule . as_str ())) . collect () ; let uncovered = uncovered_rules . len () ; total_uncovered += uncovered ; coverage_str = fmt_pct (uncovered , num_rules) ; if uncovered == 0 { uncovered_str = String :: from ("0") ; } else { uncovered_str = format ! ("<div class=\"popup-container\">\n\
                        <a href=\"javascript:void(0)\" onclick=\"spec_toggle_uncovered({item_index});\">\
                        {uncovered}</a>\n\
                        <div id=\"uncovered-{item_index}\" class=\"uncovered-rules-popup popup-hidden\">\n\
                        Uncovered rules
                        <ul>") ; for uncovered_rule in uncovered_rules { writeln ! (uncovered_str , "<li><a href=\"{html_path}#r-{uncovered_rule}\">{uncovered_rule}</a></li>") . unwrap () ; } uncovered_str . push_str ("</ul></div></div>") ; } } let indent = "&nbsp;" . repeat (level as usize * 6) ; writeln ! (table , "<tr>\n\
                <td><a href=\"{html_path}\">{indent}{number} {name}</a></td>\n\
                <td>{num_rules}</td>\n\
                <td>{num_tests_str}</td>\n\
                <td>{uncovered_str}</td>\n\
                <td>{coverage_str}</td>\n\
            </tr>" , name = ch . name ,) . unwrap () ; } let total_coverage = fmt_pct (total_uncovered , total_rules) ; writeln ! (table , "<tr>\n\
            <td><b>Total:</b></td>\n\
            <td>{total_rules}</td>\n\
            <td>{total_tests}</td>\n\
            <td>{total_uncovered}</td>\n\
            <td>{total_coverage}</td>\n\
        </tr>") . unwrap () ; table . push_str ("</table>\n") ; table }
};
}
