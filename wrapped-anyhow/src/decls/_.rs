macro_rules! _ {
    () => {
        # [cfg (anyhow_build_probe)] const _ : Option < & str > = option_env ! ("RUSTC_BOOTSTRAP") ;
    };
}

_!()