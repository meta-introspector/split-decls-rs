// Generated macro for read_vlq (function)
macro_rules! Depcrate_decoderead_vlq {
() => {
// Module: crate::decode
// Provides: {"read_vlq"}
// Dependencies: {}
pub (crate) const fn read_vlq (data : & mut & [u8]) -> Result < usize , Error > { let b = match data . split_first () { Some ((& b , rest)) => { * data = rest ; b } None => return Err (Error :: UnexpectedEnd) , } ; let mut next = b & NEXT_MASK ; let mut val = (b & VAL_MASK) as usize ; macro_rules ! step { () => { if next == 0 { return Ok (val) ; } let b = match data . split_first () { Some ((& b , rest)) => { * data = rest ; b } None => return Err (Error :: UnexpectedEnd) , } ; next = b & NEXT_MASK ; let t = (b & VAL_MASK) as usize ; val = ((val + 1) << 7) + t ; } ; } step ! () ; step ! () ; step ! () ; if next != 0 { return Err (Error :: InvalidVlq) ; } Ok (val) }
};
}
