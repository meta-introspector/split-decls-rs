// Generated macro for userinfo_should_be_percent_decode (function)
macro_rules! Depcrate_mysql_connection_urluserinfo_should_be_percent_decode {
() => {
// Module: crate::mysql::connection::url
// Provides: {"userinfo_should_be_percent_decode"}
// Dependencies: {}
# [test] fn userinfo_should_be_percent_decode () { use self :: percent_encoding :: { utf8_percent_encode , AsciiSet , CONTROLS } ; const USERINFO_ENCODE_SET : & AsciiSet = & CONTROLS . add (b' ') . add (b'"') . add (b'<') . add (b'>') . add (b'`') . add (b'#') . add (b'?') . add (b'{') . add (b'}') . add (b'/') . add (b':') . add (b';') . add (b'=') . add (b'@') . add (b'[') . add (b'\\') . add (b']') . add (b'^') . add (b'|') ; let username = "x#gfuL?4Zuj{n73m}eeJt0" ; let encoded_username = utf8_percent_encode (username , USERINFO_ENCODE_SET) ; let password = "x/gfuL?4Zuj{n73m}eeJt1" ; let encoded_password = utf8_percent_encode (password , USERINFO_ENCODE_SET) ; let db_url = format ! ("mysql://{encoded_username}:{encoded_password}@localhost/bar" ,) ; let db_url = Url :: parse (& db_url) . unwrap () ; let conn_opts = ConnectionOptions :: parse (db_url . as_str ()) . unwrap () ; let username = CString :: new (username . as_bytes ()) . unwrap () ; let password = CString :: new (password . as_bytes ()) . unwrap () ; assert_eq ! (username , conn_opts . user) ; assert_eq ! (password , conn_opts . password . unwrap ()) ; }
};
}
