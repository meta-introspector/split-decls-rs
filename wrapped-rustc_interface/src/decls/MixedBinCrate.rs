macro_rules! MixedBinCrate {
    () => {
        # [derive (Diagnostic)] # [diag (interface_mixed_bin_crate)] pub struct MixedBinCrate ;
    };
}

MixedBinCrate!();