macro_rules! parse_env {
    () => {
        # [cfg (feature = "wrap_help")] fn parse_env (var : & str) -> Option < usize > { some ! (some ! (std :: env :: var_os (var)) . to_str ()) . parse :: < usize > () . ok () }
    };
}

parse_env!()