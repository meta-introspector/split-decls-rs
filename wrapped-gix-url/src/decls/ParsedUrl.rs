macro_rules! ParsedUrl {
    () => {
        # [doc = " A minimal URL parser that extracts only what we need for git URLs."] # [doc = " This is a replacement for the `url` crate dependency."] # [derive (Debug)] pub (crate) struct ParsedUrl < 'a > { pub scheme : String , pub username : & 'a str , pub password : Option < & 'a str > , pub host : Option < String > , pub port : Option < u16 > , pub path : & 'a str , }
    };
}

ParsedUrl!()