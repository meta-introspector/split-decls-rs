macro_rules! macro_56 {
    () => {
        feature ! { #! [feature = "event"] # [cfg (linux_android)] # [allow (missing_docs)] pub mod epoll ; # [cfg (bsd)] pub mod event ; # [doc = " Event file descriptor."] # [cfg (any (linux_android , target_os = "freebsd"))] pub mod eventfd ; }
    };
}

macro_56!()