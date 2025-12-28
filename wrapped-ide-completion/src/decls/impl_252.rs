macro_rules! deps {
    () => {
        CompletionFieldsToResolve!();
    };
}

macro_rules! impl_252 {
    () => {
        deps!();
        impl CompletionFieldsToResolve { pub fn from_client_capabilities (client_capability_fields : & FxHashSet < & str >) -> Self { Self { resolve_label_details : client_capability_fields . contains ("labelDetails") , resolve_tags : client_capability_fields . contains ("tags") , resolve_detail : client_capability_fields . contains ("detail") , resolve_documentation : client_capability_fields . contains ("documentation") , resolve_filter_text : client_capability_fields . contains ("filterText") , resolve_text_edit : client_capability_fields . contains ("textEdit") , resolve_command : client_capability_fields . contains ("command") , } } pub const fn empty () -> Self { Self { resolve_label_details : false , resolve_tags : false , resolve_detail : false , resolve_documentation : false , resolve_filter_text : false , resolve_text_edit : false , resolve_command : false , } } }
    };
}

impl_252!()