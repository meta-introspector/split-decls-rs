macro_rules! deps {
    () => {
        InvalidInstructionSet!();
        UnsupportedInstructionSet!();
        BareInstructionSet!();
        MultipleInstructionSet!();
    };
}

macro_rules! parse_instruction_set_attr {
    () => {
        deps!();
        fn parse_instruction_set_attr (tcx : TyCtxt < '_ > , attr : & Attribute) -> Option < InstructionSetAttr > { let list = attr . meta_item_list () ? ; match & list [..] { [MetaItemInner :: MetaItem (set)] => { let segments = set . path . segments . iter () . map (| x | x . ident . name) . collect :: < Vec < _ > > () ; match segments . as_slice () { [sym :: arm , sym :: a32 | sym :: t32] if ! tcx . sess . target . has_thumb_interworking => { tcx . dcx () . emit_err (errors :: UnsupportedInstructionSet { span : attr . span () }) ; None } [sym :: arm , sym :: a32] => Some (InstructionSetAttr :: ArmA32) , [sym :: arm , sym :: t32] => Some (InstructionSetAttr :: ArmT32) , _ => { tcx . dcx () . emit_err (errors :: InvalidInstructionSet { span : attr . span () }) ; None } } } [] => { tcx . dcx () . emit_err (errors :: BareInstructionSet { span : attr . span () }) ; None } _ => { tcx . dcx () . emit_err (errors :: MultipleInstructionSet { span : attr . span () }) ; None } } }
    };
}

parse_instruction_set_attr!();