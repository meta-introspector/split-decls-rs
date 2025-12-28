macro_rules! deps {
    () => {
        FnCtxt!();
    };
}

macro_rules! impl_308 {
    () => {
        deps!();
        impl < 'tcx > FnCtxt < '_ , 'tcx > { # [doc = " This takes all the opaque type uses during HIR typeck. It first computes"] # [doc = " the concrete hidden type by iterating over all defining uses."] # [doc = ""] # [doc = " A use during HIR typeck is defining if all non-lifetime arguments are"] # [doc = " unique generic parameters and the hidden type does not reference any"] # [doc = " inference variables."] # [doc = ""] # [doc = " It then uses these defining uses to guide inference for all other uses."] # [instrument (level = "debug" , skip (self))] pub (super) fn handle_opaque_type_uses_next (& mut self) { let mut opaque_types : Vec < _ > = self . infcx . clone_opaque_types () ; let num_entries = self . inner . borrow_mut () . opaque_types () . num_entries () ; let prev = self . checked_opaque_types_storage_entries . replace (Some (num_entries)) ; debug_assert_eq ! (prev , None) ; for entry in & mut opaque_types { * entry = self . resolve_vars_if_possible (* entry) ; } debug ! (? opaque_types) ; self . compute_concrete_opaque_types (& opaque_types) ; self . apply_computed_concrete_opaque_types (& opaque_types) ; } }
    };
}

impl_308!();