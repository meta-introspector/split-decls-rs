macro_rules! handle_native {
    () => {
        # [doc = " If the given string is `\"native\"`, returns the host CPU name according to"] # [doc = " LLVM. Otherwise, the string is returned as-is."] fn handle_native (cpu_name : & str) -> & str { match cpu_name { "native" => get_host_cpu_name () , _ => cpu_name , } }
    };
}

handle_native!()