macro_rules! epoll_event {
    () => {
        # [doc = " Opaque types, you should use [`epoll_event`](libc::epoll_event) instead."] # [repr (C)] # [allow (non_camel_case_types)] pub struct epoll_event { _priv : () , }
    };
}

epoll_event!();