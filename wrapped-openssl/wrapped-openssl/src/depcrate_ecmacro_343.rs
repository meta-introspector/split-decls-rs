// Generated macro for macro_343 (macro)
macro_rules! Depcrate_ecmacro_343 {
() => {
// Module: crate::ec
// Provides: {"macro_343"}
// Dependencies: {}
foreign_type_and_impl_send_sync ! { type CType = ffi :: EC_GROUP ; fn drop = ffi :: EC_GROUP_free ; # [doc = " Describes the curve"] # [doc = ""] # [doc = " A curve can be of the named curve type.  These curves can be discovered"] # [doc = " using openssl binary `openssl ecparam -list_curves`.  Other operations"] # [doc = " are available in the [wiki].  These named curves are available in the"] # [doc = " [`Nid`] module."] # [doc = ""] # [doc = " Curves can also be generated using prime field parameters or a binary field."] # [doc = ""] # [doc = " Prime fields use the formula `y^2 mod p = x^3 + ax + b mod p`.  Binary"] # [doc = " fields use the formula `y^2 + xy = x^3 + ax^2 + b`.  Named curves have"] # [doc = " assured security.  To prevent accidental vulnerabilities, they should"] # [doc = " be preferred."] # [doc = ""] # [doc = " [wiki]: https://wiki.openssl.org/index.php/Command_Line_Elliptic_Curve_Operations"] # [doc = " [`Nid`]: ../nid/index.html"] pub struct EcGroup ; # [doc = " Reference to [`EcGroup`]"] # [doc = ""] # [doc = " [`EcGroup`]: struct.EcGroup.html"] pub struct EcGroupRef ; }
};
}
