macro_rules! DiagnosticItems {
    () => {
        # [derive (Debug , Default)] pub struct DiagnosticItems { pub id_to_name : DefIdMap < Symbol > , pub name_to_id : FxIndexMap < Symbol , DefId > , }
    };
}

DiagnosticItems!()