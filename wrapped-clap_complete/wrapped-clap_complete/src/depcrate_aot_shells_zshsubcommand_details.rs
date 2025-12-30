// Generated macro for subcommand_details (function)
macro_rules! Depcrate_aot_shells_zshsubcommand_details {
() => {
// Module: crate::aot::shells::zsh
// Provides: {"subcommand_details"}
// Dependencies: {}
fn subcommand_details (p : & Command) -> String { debug ! ("subcommand_details") ; let bin_name = p . get_bin_name () . expect ("crate::generate should have set the bin_name") ; let mut ret = vec ! [] ; let parent_text = format ! ("\
(( $+functions[_{bin_name_underscore}_commands] )) ||
_{bin_name_underscore}_commands() {{
    local commands; commands=({subcommands_and_args})
    _describe -t commands '{bin_name} commands' commands \"$@\"
}}" , bin_name_underscore = bin_name . replace (' ' , "__") , bin_name = bin_name , subcommands_and_args = subcommands_of (p)) ; ret . push (parent_text) ; let mut all_subcommand_bins : Vec < _ > = utils :: all_subcommands (p) . into_iter () . map (| (_sc_name , bin_name) | bin_name) . collect () ; all_subcommand_bins . sort () ; all_subcommand_bins . dedup () ; for bin_name in & all_subcommand_bins { debug ! ("subcommand_details:iter: bin_name={bin_name}") ; ret . push (format ! ("\
(( $+functions[_{bin_name_underscore}_commands] )) ||
_{bin_name_underscore}_commands() {{
    local commands; commands=({subcommands_and_args})
    _describe -t commands '{bin_name} commands' commands \"$@\"
}}" , bin_name_underscore = bin_name . replace (' ' , "__") , bin_name = bin_name , subcommands_and_args = subcommands_of (parser_of (p , bin_name) . expect (INTERNAL_ERROR_MSG)))) ; } ret . join ("\n") }
};
}
