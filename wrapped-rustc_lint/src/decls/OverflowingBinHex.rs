macro_rules! deps {
    () => {
        OverflowingBinHexSub!();
        OverflowingBinHexSign!();
        OverflowingBinHexSignBitSub!();
    };
}

macro_rules! OverflowingBinHex {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (lint_overflowing_bin_hex)] pub (crate) struct OverflowingBinHex < 'a > { pub ty : & 'a str , pub lit : String , pub dec : u128 , pub actually : String , # [subdiagnostic] pub sign : OverflowingBinHexSign , # [subdiagnostic] pub sub : Option < OverflowingBinHexSub < 'a > > , # [subdiagnostic] pub sign_bit_sub : Option < OverflowingBinHexSignBitSub < 'a > > , }
    };
}

OverflowingBinHex!()