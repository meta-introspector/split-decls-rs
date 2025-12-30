// Generated macro for deduplicate_identities (function)
macro_rules! Depcrate_hours_corededuplicate_identities {
() => {
// Module: crate::hours::core
// Provides: {"deduplicate_identities"}
// Dependencies: {}
pub fn deduplicate_identities (persons : & [WorkByEmail]) -> Vec < WorkByPerson > { let mut email_to_index = HashMap :: < & 'static BStr , usize > :: with_capacity (persons . len ()) ; let mut name_to_index = HashMap :: < & 'static BStr , usize > :: with_capacity (persons . len ()) ; let mut out = Vec :: < WorkByPerson > :: with_capacity (persons . len ()) ; for person_by_email in persons { match email_to_index . entry (person_by_email . email) { Entry :: Occupied (email_entry) => { out [* email_entry . get ()] . merge (person_by_email) ; name_to_index . insert (person_by_email . name , * email_entry . get ()) ; } Entry :: Vacant (email_entry) => match name_to_index . entry (person_by_email . name) { Entry :: Occupied (name_entry) => { out [* name_entry . get ()] . merge (person_by_email) ; email_entry . insert (* name_entry . get ()) ; } Entry :: Vacant (name_entry) => { let idx = out . len () ; name_entry . insert (idx) ; email_entry . insert (idx) ; out . push (person_by_email . into ()) ; } } , } } out }
};
}
