macro_rules! deps {
    () => {
        PasswordClient!();
    };
}

macro_rules! PasswordClientBuilder {
    () => {
        deps!();
        # [doc = " Builds a [`PasswordClient`] from the supplied challenges; create via"] # [doc = " [`PasswordClient::builder`]."] # [doc = ""] # [doc = " Often you can just use [`PasswordClient`]'s [`TryFrom`] implementations"] # [doc = " to convert from a parsed challenge ([`crate::ChallengeRef`]) or"] # [doc = " unparsed challenges (`str`, [`http::header::HeaderValue`], or"] # [doc = " [`http::header::GetAll`])."] # [doc = ""] # [doc = " The builder allows more flexibility. For example, if you are using a HTTP"] # [doc = " library which is not based on a `http` crate, you might need to create"] # [doc = " a `PasswordClient` from an iterator over multiple `WWW-Authenticate`"] # [doc = " headers. You can feed each to [`PasswordClientBuilder::challenges`]."] # [doc = ""] # [doc = " Prefers `Digest` over `Basic`, consistent with the [RFC 7235 section"] # [doc = " 2.1](https://datatracker.ietf.org/doc/html/rfc7235#section-2.1) advice"] # [doc = " for a user-agent to pick the most secure auth-scheme it understands."] # [doc = ""] # [doc = " When there are multiple `Digest` challenges, currently uses the first,"] # [doc = " consistent with the [RFC 7616 section"] # [doc = " 3.7](https://datatracker.ietf.org/doc/html/rfc7616#section-3.7)"] # [doc = " advice to \"use the first challenge it supports, unless a local policy"] # [doc = " dictates otherwise\". In the future, it may prioritize by algorithm."] # [doc = ""] # [doc = " Ignores parse errors as long as there's at least one parseable, supported"] # [doc = " challenge."] # [doc = ""] # [doc = " ## Example"] # [doc = ""] # [cfg_attr (feature = "digest" , doc = r##"
```rust
use http_auth::PasswordClient;
let client = PasswordClient::builder()
    .challenges("UnsupportedSchemeA, Basic realm=\"foo\", UnsupportedSchemeB")
    .challenges("Digest \
                 realm=\"http-auth@example.org\", \
                 qop=\"auth, auth-int\", \
                 algorithm=MD5, \
                 nonce=\"7ypf/xlj9XXwfDPEoM4URrv/xwf94BcCAzFZH4GiTo0v\", \
                 opaque=\"FQhe/qaU925kfnzjCev0ciny7QMkPqMAFRtzCUYo5tdS\"")
    .build()
    .unwrap();
assert!(matches!(client, PasswordClient::Digest(_)));
```
"##)] # [derive (Default)] pub struct PasswordClientBuilder (# [doc = " The current result:"] # [doc = " *   `Some(Ok(_))` if there is a suitable client."] # [doc = " *   `Some(Err(_))` if there is no suitable client and has been a parse error."] # [doc = " *   `None` otherwise."] Option < Result < PasswordClient , String > > ,) ;
    };
}

PasswordClientBuilder!();