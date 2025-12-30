// Generated macro for vals_for (function)
macro_rules! Depcrate_aot_shells_bashvals_for {
() => {
// Module: crate::aot::shells::bash
// Provides: {"vals_for"}
// Dependencies: {}
fn vals_for (o : & Arg) -> String { debug ! ("vals_for: o={}" , o . get_id ()) ; if let Some (vals) = utils :: possible_values (o) { format ! ("$(compgen -W \"{}\" -- \"${{cur}}\")" , vals . iter () . filter (| pv | ! pv . is_hide_set ()) . map (| n | n . get_name ()) . collect ::< Vec < _ >> () . join (" ")) } else if o . get_value_hint () == ValueHint :: DirPath { String :: from ("") } else if o . get_value_hint () == ValueHint :: Other { String :: from ("\"${cur}\"") } else { String :: from ("$(compgen -f \"${cur}\")") } }
};
}
