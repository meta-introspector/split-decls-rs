// Generated macro for decode_from_iter (function)
macro_rules! Depcratedecode_from_iter {
() => {
// Module: crate
// Provides: {"decode_from_iter"}
// Dependencies: {}
fn decode_from_iter (decoded : & mut Vec < u8 > , iter : & mut slice :: Iter < u8 > , variant : Variant) -> bool { macro_rules ! err { () => { return false } } macro_rules ! next { () => { match iter . next () { Some (a) => * a , None => err ! () } } } macro_rules ! next_cont { () => { { let byte = next ! () ; if (byte) & ! CONT_MASK == TAG_CONT_U8 { byte } else { err ! () } } } } loop { let first = match iter . next () { Some (& b) => b , None => return true } ; if variant == Variant :: Java && first == 0 { err ! () ; } else if first < 128 { decoded . push (first) ; } else if first == 0xc0 && variant == Variant :: Java { match next ! () { 0x80 => decoded . push (0) , _ => err ! () , } } else { let w = utf8_char_width (first) ; let second = next_cont ! () ; match w { 2 => { decoded . extend ([first , second] . iter () . cloned ()) ; } 3 => { let third = next_cont ! () ; match (first , second) { (0xE0 , 0xA0 ..= 0xBF) | (0xE1 ..= 0xEC , 0x80 ..= 0xBF) | (0xED , 0x80 ..= 0x9F) | (0xEE ..= 0xEF , 0x80 ..= 0xBF) => { decoded . extend ([first , second , third] . iter () . cloned ()) } (0xED , 0xA0 ..= 0xAF) => { if next ! () != 0xED { err ! () } let fifth = next_cont ! () ; if fifth < 0xB0 || 0xBF < fifth { err ! () } let sixth = next_cont ! () ; let s = dec_surrogates (second , third , fifth , sixth) ; decoded . extend (s . iter () . cloned ()) ; } _ => err ! () } } _ => err ! () } } } }
};
}
