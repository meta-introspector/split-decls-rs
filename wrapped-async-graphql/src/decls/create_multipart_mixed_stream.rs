macro_rules! deps {
    () => {
        Response!();
    };
}

macro_rules! create_multipart_mixed_stream {
    () => {
        deps!();
        # [doc = " Create a stream for `multipart/mixed` responses."] # [doc = ""] # [doc = " Reference: <https://www.apollographql.com/docs/router/executing-operations/subscription-multipart-protocol/>"] pub fn create_multipart_mixed_stream < 'a > (input : impl Stream < Item = Response > + Send + Unpin + 'a , heartbeat_interval : Duration ,) -> BoxStream < 'a , Bytes > { let mut input = input . fuse () ; let mut heartbeat_timer = Delay :: new (heartbeat_interval) . fuse () ; async_stream :: stream ! { loop { futures_util :: select ! { item = input . next () => { match item { Some (resp) => { let data = BytesMut :: new () ; let mut writer = data . writer () ; if serde_json :: to_writer (& mut writer , & resp) . is_err () { continue ; } yield PART_HEADER . clone () ; yield writer . into_inner () . freeze () ; yield CRLF . clone () ; } None => break , } } _ = heartbeat_timer => { heartbeat_timer = Delay :: new (heartbeat_interval) . fuse () ; yield PART_HEADER . clone () ; yield HEARTBEAT . clone () ; } } } yield EOF . clone () ; } . boxed () }
    };
}

create_multipart_mixed_stream!();