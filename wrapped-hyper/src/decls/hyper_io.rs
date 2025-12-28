macro_rules! hyper_io {
    () => {
        # [doc = " A read/write handle for a specific connection."] # [doc = ""] # [doc = " This owns a specific TCP or TLS connection for the lifetime of"] # [doc = " that connection. It contains a read and write callback, as well as a"] # [doc = " void *userdata. Typically the userdata will point to a struct"] # [doc = " containing a file descriptor and a TLS context."] # [doc = ""] # [doc = " Methods:"] # [doc = ""] # [doc = " - hyper_io_new:          Create a new IO type used to represent a transport."] # [doc = " - hyper_io_set_read:     Set the read function for this IO transport."] # [doc = " - hyper_io_set_write:    Set the write function for this IO transport."] # [doc = " - hyper_io_set_userdata: Set the user data pointer for this IO to some value."] # [doc = " - hyper_io_free:         Free an IO handle."] pub struct hyper_io { read : hyper_io_read_callback , write : hyper_io_write_callback , userdata : * mut c_void , }
    };
}

hyper_io!()