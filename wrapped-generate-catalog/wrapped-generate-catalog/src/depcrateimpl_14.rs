// Generated macro for impl_14 (impl)
macro_rules! Depcrateimpl_14 {
() => {
// Module: crate
// Provides: {"impl_14"}
// Dependencies: {}
impl Algorithm { pub fn emit_rust (& self , mut writer : impl Write) -> Result < () , Error > { let const_name = NAME_REPLACE_REGEX . replace_all (& self . parameters . name , "_") ; let int_ty = int_type_for (self . parameters . width) ; let poly_rev = reverse_bits (self . parameters . poly , self . parameters . width) ; let Self { parameters : Parameters { width , poly , init , refin , refout , xorout , check , residue , name } , url , aliases , } = & self ; writeln ! (writer , r#"/// # [`{name}`][1]
///
/// - `width`: `{width}` bits
/// - `poly`: `0x{poly:x}` (reversed: `0x{poly_rev:x}`)
/// - `init`: `0x{init:x}`
/// - `refin`: `{refin:?}`
/// - `refout`: `{refout:?}`
/// - `xorout`: `0x{xorout:x}`
/// - `check`: `0x{check:x}`
/// - `residue`: `0x{residue:x}`
///
/// [1]: {url}
pub const {const_name}: Algorithm<{int_ty}> = Algorithm {{
    width: {width},
    poly: 0x{poly:x},
    init: 0x{init:x},
    refin: {refin:?},
    refout: {refout:?},
    xorout: 0x{xorout:x},
    check: 0x{check:x},
    residue: 0x{residue:x}
}};
"#) ? ; for alias in aliases { writeln ! (writer , r#"/// Alias for [`{const_name}`].
pub const {alias}: Algorithm<{int_ty}> = {const_name};
"#) ? ; } Ok (()) } }
};
}
