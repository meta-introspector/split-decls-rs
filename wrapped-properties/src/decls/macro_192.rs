macro_rules! macro_192 {
    () => {
        make_binary_property ! { name : "Join_Control" ; short_name : "Join_C" ; ident : JoinControl ; data_marker : crate :: provider :: PropertyBinaryJoinControlV1 ; singleton : SINGLETON_PROPERTY_BINARY_JOIN_CONTROL_V1 ; # [doc = " Format control characters which have specific functions for control of cursive joining"] # [doc = " and ligation."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::JoinControl;"] # [doc = ""] # [doc = " let join_control = CodePointSetData::new::<JoinControl>();"] # [doc = ""] # [doc = " assert!(join_control.contains('\\u{200C}'));  // ZERO WIDTH NON-JOINER"] # [doc = " assert!(join_control.contains('\\u{200D}'));  // ZERO WIDTH JOINER"] # [doc = " assert!(!join_control.contains('\\u{200E}'));"] # [doc = " ```"] }
    };
}

macro_192!()