// Generated macro for tests (module)
macro_rules! Depcrate_mailmaptests {
() => {
// Module: crate::mailmap
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn smoke () { let sig_name = "name" ; let sig_email = "email" ; let sig = t ! (Signature :: now (sig_name , sig_email)) ; let mut mm = t ! (Mailmap :: new ()) ; let mailmapped_sig = t ! (mm . resolve_signature (& sig)) ; assert_eq ! (mailmapped_sig . name () , Some (sig_name)) ; assert_eq ! (mailmapped_sig . email () , Some (sig_email)) ; t ! (mm . add_entry (None , None , None , sig_email)) ; t ! (mm . add_entry (Some ("real name") , Some ("real@email") , Some (sig_name) , sig_email ,)) ; let mailmapped_sig = t ! (mm . resolve_signature (& sig)) ; assert_eq ! (mailmapped_sig . name () , Some ("real name")) ; assert_eq ! (mailmapped_sig . email () , Some ("real@email")) ; } # [test] fn from_buffer () { let buf = "<prøper@emæil> <email>" ; let mm = t ! (Mailmap :: from_buffer (& buf)) ; let sig = t ! (Signature :: now ("name" , "email")) ; let mailmapped_sig = t ! (mm . resolve_signature (& sig)) ; assert_eq ! (mailmapped_sig . name () , Some ("name")) ; assert_eq ! (mailmapped_sig . email () , Some ("prøper@emæil")) ; } }
};
}
