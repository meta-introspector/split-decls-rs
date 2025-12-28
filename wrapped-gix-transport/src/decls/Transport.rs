macro_rules! deps {
    () => {
        WriteMode!();
        SetServiceResponse!();
        MessageKind!();
        TransportWithoutIO!();
        Protocol!();
        Error!();
        Service!();
        RequestWriter!();
    };
}

macro_rules! Transport {
    () => {
        deps!();
        # [doc = " All methods provided here must be called in the correct order according to the [communication protocol][Protocol]"] # [doc = " used to connect to them."] # [doc = " It does, however, know just enough to be able to provide a higher-level interface than would otherwise be possible."] # [doc = " Thus the consumer of this trait will not have to deal with packet lines at all."] # [doc = " **Note that**  whenever a `Read` trait or `Write` trait is produced, it must be exhausted."] pub trait Transport : TransportWithoutIO { # [doc = " Initiate connection to the given service and send the given `extra_parameters` along with it."] # [doc = ""] # [doc = " `extra_parameters` are interpreted as `key=value` pairs if the second parameter is `Some` or as `key`"] # [doc = " if it is None."] # [doc = ""] # [doc = " Returns the service capabilities according to the actual [Protocol] it supports,"] # [doc = " and possibly a list of refs to be obtained."] # [doc = " This means that asking for an unsupported protocol might result in a protocol downgrade to the given one"] # [doc = " if [`TransportWithoutIO::supported_protocol_versions()`] includes it."] # [doc = " Exhaust the returned [`BufReader`][SetServiceResponse::refs] for a list of references in case of protocol V1"] # [doc = " before making another request."] fn handshake < 'a > (& mut self , service : Service , extra_parameters : & 'a [(& 'a str , Option < & 'a str >)] ,) -> Result < SetServiceResponse < '_ > , Error > ; # [doc = " Get a writer for sending data and obtaining the response. It can be configured in various ways"] # [doc = " to support the task at hand."] # [doc = " `write_mode` determines how calls to the `write(…)` method are interpreted, and `on_into_read` determines"] # [doc = " which message to write when the writer is turned into the response reader using [`into_read()`][RequestWriter::into_read()]."] # [doc = " If `trace` is `true`, then all packetlines written and received will be traced using facilities provided by the `gix_trace` crate."] fn request (& mut self , write_mode : WriteMode , on_into_read : MessageKind , trace : bool ,) -> Result < RequestWriter < '_ > , Error > ; }
    };
}

Transport!()