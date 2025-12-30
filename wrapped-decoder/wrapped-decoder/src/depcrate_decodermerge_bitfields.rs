// Generated macro for merge_bitfields (function)
macro_rules! Depcrate_decodermerge_bitfields {
() => {
// Module: crate::decoder
// Provides: {"merge_bitfields"}
// Dependencies: {}
# [doc = " Note that this will not change the Bitfield params in place, i.e. if `params` was sorted before"] # [doc = " a call to this function, it won't be afterwards."] fn merge_bitfields (params : & mut Vec < Parameter >) { if params . is_empty () { return ; } let mut merged_bitfields = Vec :: new () ; let max_index : usize = * params . iter () . map (| param | & param . index) . max () . unwrap () ; for index in 0 ..= max_index { let mut bitfields_with_index = params . iter () . filter (| param | matches ! ((param . index , & param . ty) , (i , Type :: BitField (_)) if i == index) ,) . peekable () ; if bitfields_with_index . peek () . is_some () { let (smallest , largest) = get_max_bitfield_range (bitfields_with_index) . unwrap () ; merged_bitfields . push (Parameter { index , ty : Type :: BitField (Range { start : smallest , end : largest , }) , hint : None , }) ; let mut i = 0 ; while i != params . len () { match & params [i] . ty { Type :: BitField (_) => { if params [i] . index == index { params . remove (i) ; } else { i += 1 ; } } _ => { i += 1 ; } } } } } params . append (& mut merged_bitfields) ; }
};
}
