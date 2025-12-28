macro_rules! Local {
    () => {
        # [doc = " A single local definition."] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] pub struct Local { pub (crate) parent : DefWithBodyId , pub (crate) binding_id : BindingId , }
    };
}

Local!();