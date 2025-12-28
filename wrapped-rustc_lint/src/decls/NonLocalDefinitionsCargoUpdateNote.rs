macro_rules! NonLocalDefinitionsCargoUpdateNote {
    () => {
        # [derive (Subdiagnostic)] # [note (lint_non_local_definitions_cargo_update)] pub (crate) struct NonLocalDefinitionsCargoUpdateNote { pub macro_kind : & 'static str , pub macro_name : Symbol , pub crate_name : Symbol , }
    };
}

NonLocalDefinitionsCargoUpdateNote!()