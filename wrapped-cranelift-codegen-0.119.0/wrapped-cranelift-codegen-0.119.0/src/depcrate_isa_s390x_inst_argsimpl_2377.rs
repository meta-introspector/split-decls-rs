// Generated macro for impl_2377 (impl)
macro_rules! Depcrate_isa_s390x_inst_argsimpl_2377 {
() => {
// Module: crate::isa::s390x::inst::args
// Provides: {"impl_2377"}
// Dependencies: {}
impl PrettyPrint for MemArg { fn pretty_print (& self , _ : u8) -> String { match self { & MemArg :: BXD12 { base , index , disp , .. } => { if base != zero_reg () { if index != zero_reg () { format ! ("{}({},{})" , disp . pretty_print_default () , show_reg (index) , show_reg (base) ,) } else { format ! ("{}({})" , disp . pretty_print_default () , show_reg (base)) } } else { if index != zero_reg () { format ! ("{}({},)" , disp . pretty_print_default () , show_reg (index)) } else { format ! ("{}" , disp . pretty_print_default ()) } } } & MemArg :: BXD20 { base , index , disp , .. } => { if base != zero_reg () { if index != zero_reg () { format ! ("{}({},{})" , disp . pretty_print_default () , show_reg (index) , show_reg (base) ,) } else { format ! ("{}({})" , disp . pretty_print_default () , show_reg (base)) } } else { if index != zero_reg () { format ! ("{}({},)" , disp . pretty_print_default () , show_reg (index)) } else { format ! ("{}" , disp . pretty_print_default ()) } } } & MemArg :: Label { target } => target . to_string () , & MemArg :: Symbol { ref name , offset , .. } => format ! ("{} + {}" , name . display (None) , offset) , & MemArg :: InitialSPOffset { .. } | & MemArg :: NominalSPOffset { .. } | & MemArg :: SlotOffset { .. } | & MemArg :: RegOffset { .. } => { panic ! ("Unexpected pseudo mem-arg mode (stack-offset or generic reg-offset)!") } } } }
};
}
