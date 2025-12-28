macro_rules! ContainerHandle {
    () => {
        # [doc = " A handle to a running container."] # [doc = ""] # [doc = " You can use this to interact with the container."] pub struct ContainerHandle { # [doc = " The name of the container."] name : String , # [doc = " The IP address of the container."] # [doc = ""] # [doc = " NOTE: This is currently unused, but may be useful so I left it in."] # [doc = " This can only be used on Linux. macOS and Windows docker doesn't allow"] # [doc = " direct connection to the container."] pub ip_address : String , # [doc = " Port mappings of `container_port` to `host_port` for ports exposed via EXPOSE."] pub port_mappings : HashMap < u16 , u16 > , }
    };
}

ContainerHandle!();