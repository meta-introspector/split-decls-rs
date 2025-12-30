// Generated macro for json_to_intrinsic (function)
macro_rules! Depcrate_arm_json_parserjson_to_intrinsic {
() => {
// Module: crate::arm::json_parser
// Provides: {"json_to_intrinsic"}
// Dependencies: {}
fn json_to_intrinsic (mut intr : JsonIntrinsic , target : & str ,) -> Result < Intrinsic < ArmIntrinsicType > , Box < dyn std :: error :: Error > > { let name = intr . name . replace (['[' , ']'] , "") ; let results = ArmIntrinsicType :: from_c (& intr . return_type . value , target) ? ; let args = intr . arguments . into_iter () . enumerate () . map (| (i , arg) | { let (type_name , arg_name) = Argument :: < ArmIntrinsicType > :: type_and_name_from_c (& arg) ; let metadata = intr . args_prep . as_mut () ; let metadata = metadata . and_then (| a | a . remove (arg_name)) ; let arg_prep : Option < ArgPrep > = metadata . and_then (| a | a . try_into () . ok ()) ; let constraint : Option < Constraint > = arg_prep . and_then (| a | a . try_into () . ok ()) ; let ty = ArmIntrinsicType :: from_c (type_name , target) . unwrap_or_else (| _ | panic ! ("Failed to parse argument '{arg}'")) ; let mut arg = Argument :: < ArmIntrinsicType > :: new (i , String :: from (arg_name) , ty , constraint) ; let IntrinsicType { ref mut constant , .. } = arg . ty . data ; if arg . name . starts_with ("imm") { * constant = true } arg }) . collect () ; let arguments = ArgumentList :: < ArmIntrinsicType > { args } ; Ok (Intrinsic { name , arguments , results , arch_tags : intr . architectures , }) }
};
}
