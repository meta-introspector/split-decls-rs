// Generated macro for EphemeralSecret (struct)
macro_rules! Depcrate_ecdhEphemeralSecret {
() => {
// Module: crate::ecdh
// Provides: {"EphemeralSecret"}
// Dependencies: {}
# [doc = " Ephemeral Diffie-Hellman Secret."] # [doc = ""] # [doc = " These are ephemeral \"secret key\" values which are deliberately designed"] # [doc = " to avoid being persisted."] # [doc = ""] # [doc = " To perform an ephemeral Diffie-Hellman exchange, do the following:"] # [doc = ""] # [doc = " - Have each participant generate an [`EphemeralSecret`] value"] # [doc = " - Compute the [`PublicKey`] for that value"] # [doc = " - Have each peer provide their [`PublicKey`] to their counterpart"] # [doc = " - Use [`EphemeralSecret`] and the other participant's [`PublicKey`]"] # [doc = "   to compute a [`SharedSecret`] value."] # [doc = ""] # [doc = " # ⚠\u{fe0f} SECURITY WARNING ⚠\u{fe0f}"] # [doc = ""] # [doc = " Ephemeral Diffie-Hellman exchanges are unauthenticated and without a"] # [doc = " further authentication step are trivially vulnerable to man-in-the-middle"] # [doc = " attacks!"] # [doc = ""] # [doc = " These exchanges should be performed in the context of a protocol which"] # [doc = " takes further steps to authenticate the peers in a key exchange."] pub struct EphemeralSecret < C > where C : CurveArithmetic , { scalar : NonZeroScalar < C > , }
};
}
