macro_rules! deps {
    () => {
        CrlfRoundTripCheck!();
        Driver!();
        Configuration!();
    };
}

macro_rules! Options {
    () => {
        deps!();
        # [doc = " Additional configuration for the filter pipeline."] # [derive (Default , Clone)] pub struct Options { # [doc = " Available (external) driver programs to invoke if attributes for path configure them."] pub drivers : Vec < Driver > , # [doc = " Global options to configure end-of-line conversions, to worktree or to git."] pub eol_config : eol :: Configuration , # [doc = " How to perform round-trip checks during end-of-line conversions to git."] pub crlf_roundtrip_check : CrlfRoundTripCheck , # [doc = " All worktree encodings for round-trip checks should be performed."] pub encodings_with_roundtrip_check : Vec < & 'static encoding_rs :: Encoding > , # [doc = " The object hash to use when applying the `ident` filter."] pub object_hash : gix_hash :: Kind , }
    };
}

Options!();