// Generated macro for tests (module)
macro_rules! Depcrate_digesttests {
() => {
// Module: crate::digest
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use pretty_assertions :: assert_eq ; # [doc = " Tests the example from [RFC 7616 section 3.9.1: SHA-256 and"] # [doc = " MD5](https://datatracker.ietf.org/doc/html/rfc7616#section-3.9.1)."] # [test] fn sha256_and_md5 () { let www_authenticate = "\
            Digest \
            realm=\"http-auth@example.org\", \
            qop=\"auth, auth-int\", \
            algorithm=SHA-256, \
            nonce=\"7ypf/xlj9XXwfDPEoM4URrv/xwf94BcCAzFZH4GiTo0v\", \
            opaque=\"FQhe/qaU925kfnzjCev0ciny7QMkPqMAFRtzCUYo5tdS\", \
            Digest \
            realm=\"http-auth@example.org\", \
            qop=\"auth, auth-int\", \
            algorithm=MD5, \
            nonce=\"7ypf/xlj9XXwfDPEoM4URrv/xwf94BcCAzFZH4GiTo0v\", \
            opaque=\"FQhe/qaU925kfnzjCev0ciny7QMkPqMAFRtzCUYo5tdS\"" ; let challenges = dbg ! (crate :: parse_challenges (www_authenticate) . unwrap ()) ; assert_eq ! (challenges . len () , 2) ; let ctxs : Result < Vec < _ > , _ > = challenges . iter () . map (DigestClient :: try_from) . collect () ; let mut ctxs = dbg ! (ctxs . unwrap ()) ; assert_eq ! (ctxs [1] . realm () , "http-auth@example.org") ; assert_eq ! (ctxs [1] . domain () , "") ; assert_eq ! (ctxs [1] . nonce () , "7ypf/xlj9XXwfDPEoM4URrv/xwf94BcCAzFZH4GiTo0v") ; assert_eq ! (ctxs [1] . opaque () , Some ("FQhe/qaU925kfnzjCev0ciny7QMkPqMAFRtzCUYo5tdS")) ; assert_eq ! (ctxs [1] . stale () , false) ; assert_eq ! (ctxs [1] . algorithm () , Algorithm :: Md5) ; assert_eq ! (ctxs [1] . qop () . 0 , (Qop :: Auth as u8) | (Qop :: AuthInt as u8)) ; assert_eq ! (ctxs [1] . nonce_count () , 0) ; let params = crate :: PasswordParams { username : "Mufasa" , password : "Circle of Life" , uri : "/dir/index.html" , body : None , method : "GET" , } ; assert_eq ! (& mut ctxs [0] . respond_with_testing_cnonce (& params , "f2/wE4q74E6zIJEtWaHKaf5wv/H5QzzpXusqGemxURZJ") . unwrap () , "Digest username=\"Mufasa\", \
                    realm=\"http-auth@example.org\", \
                    uri=\"/dir/index.html\", \
                    nonce=\"7ypf/xlj9XXwfDPEoM4URrv/xwf94BcCAzFZH4GiTo0v\", \
                    algorithm=SHA-256, \
                    nc=00000001, \
                    cnonce=\"f2/wE4q74E6zIJEtWaHKaf5wv/H5QzzpXusqGemxURZJ\", \
                    qop=auth, \
                    response=\"753927fa0e85d155564e2e272a28d1802ca10daf4496794697cf8db5856cb6c1\", \
                    opaque=\"FQhe/qaU925kfnzjCev0ciny7QMkPqMAFRtzCUYo5tdS\"") ; assert_eq ! (ctxs [0] . nc , 1) ; assert_eq ! (& mut ctxs [1] . respond_with_testing_cnonce (& params , "f2/wE4q74E6zIJEtWaHKaf5wv/H5QzzpXusqGemxURZJ") . unwrap () , "Digest username=\"Mufasa\", \
                    realm=\"http-auth@example.org\", \
                    uri=\"/dir/index.html\", \
                    nonce=\"7ypf/xlj9XXwfDPEoM4URrv/xwf94BcCAzFZH4GiTo0v\", \
                    algorithm=MD5, \
                    nc=00000001, \
                    cnonce=\"f2/wE4q74E6zIJEtWaHKaf5wv/H5QzzpXusqGemxURZJ\", \
                    qop=auth, \
                    response=\"8ca523f5e9506fed4657c9700eebdbec\", \
                    opaque=\"FQhe/qaU925kfnzjCev0ciny7QMkPqMAFRtzCUYo5tdS\"") ; assert_eq ! (ctxs [1] . nc , 1) ; } # [doc = " Tests a made-up example with `MD5-sess`. There's no example in the RFC,"] # [doc = " and these values haven't been tested against any other implementation."] # [doc = " But having the test here ensures we don't accidentally change the"] # [doc = " algorithm."] # [test] fn md5_sess () { let www_authenticate = "\
            Digest \
            realm=\"http-auth@example.org\", \
            qop=\"auth, auth-int\", \
            algorithm=MD5-sess, \
            nonce=\"7ypf/xlj9XXwfDPEoM4URrv/xwf94BcCAzFZH4GiTo0v\", \
            opaque=\"FQhe/qaU925kfnzjCev0ciny7QMkPqMAFRtzCUYo5tdS\"" ; let challenges = dbg ! (crate :: parse_challenges (www_authenticate) . unwrap ()) ; assert_eq ! (challenges . len () , 1) ; let ctxs : Result < Vec < _ > , _ > = challenges . iter () . map (DigestClient :: try_from) . collect () ; let mut ctxs = dbg ! (ctxs . unwrap ()) ; assert_eq ! (ctxs [0] . realm () , "http-auth@example.org") ; assert_eq ! (ctxs [0] . domain () , "") ; assert_eq ! (ctxs [0] . nonce () , "7ypf/xlj9XXwfDPEoM4URrv/xwf94BcCAzFZH4GiTo0v") ; assert_eq ! (ctxs [0] . opaque () , Some ("FQhe/qaU925kfnzjCev0ciny7QMkPqMAFRtzCUYo5tdS")) ; assert_eq ! (ctxs [0] . stale () , false) ; assert_eq ! (ctxs [0] . algorithm () , Algorithm :: Md5) ; assert_eq ! (ctxs [0] . session () , true) ; assert_eq ! (ctxs [0] . qop () . 0 , (Qop :: Auth as u8) | (Qop :: AuthInt as u8)) ; assert_eq ! (ctxs [0] . nonce_count () , 0) ; let params = crate :: PasswordParams { username : "Mufasa" , password : "Circle of Life" , uri : "/dir/index.html" , body : None , method : "GET" , } ; assert_eq ! (& mut ctxs [0] . respond_with_testing_cnonce (& params , "f2/wE4q74E6zIJEtWaHKaf5wv/H5QzzpXusqGemxURZJ") . unwrap () , "Digest username=\"Mufasa\", \
                    realm=\"http-auth@example.org\", \
                    uri=\"/dir/index.html\", \
                    nonce=\"7ypf/xlj9XXwfDPEoM4URrv/xwf94BcCAzFZH4GiTo0v\", \
                    algorithm=MD5-sess, \
                    nc=00000001, \
                    cnonce=\"f2/wE4q74E6zIJEtWaHKaf5wv/H5QzzpXusqGemxURZJ\", \
                    qop=auth, \
                    response=\"e783283f46242139c486a698fec7211d\", \
                    opaque=\"FQhe/qaU925kfnzjCev0ciny7QMkPqMAFRtzCUYo5tdS\"") ; assert_eq ! (ctxs [0] . nc , 1) ; } # [doc = " Tests the example from [RFC 7616 section 3.9.2: SHA-512-256, Charset, and"] # [doc = " Userhash](https://datatracker.ietf.org/doc/html/rfc7616#section-3.9.2)."] # [test] fn sha512_256_charset () { let www_authenticate = "\
            Digest \
            realm=\"api@example.org\", \
            qop=\"auth\", \
            algorithm=SHA-512-256, \
            nonce=\"5TsQWLVdgBdmrQ0XsxbDODV+57QdFR34I9HAbC/RVvkK\", \
            opaque=\"HRPCssKJSGjCrkzDg8OhwpzCiGPChXYjwrI2QmXDnsOS\", \
            charset=UTF-8, \
            userhash=true" ; let challenges = dbg ! (crate :: parse_challenges (www_authenticate) . unwrap ()) ; assert_eq ! (challenges . len () , 1) ; let ctxs : Result < Vec < _ > , _ > = challenges . iter () . map (DigestClient :: try_from) . collect () ; let mut ctxs = dbg ! (ctxs . unwrap ()) ; assert_eq ! (ctxs . len () , 1) ; assert_eq ! (ctxs [0] . realm () , "api@example.org") ; assert_eq ! (ctxs [0] . domain () , "") ; assert_eq ! (ctxs [0] . nonce () , "5TsQWLVdgBdmrQ0XsxbDODV+57QdFR34I9HAbC/RVvkK") ; assert_eq ! (ctxs [0] . opaque () , Some ("HRPCssKJSGjCrkzDg8OhwpzCiGPChXYjwrI2QmXDnsOS")) ; assert_eq ! (ctxs [0] . stale , false) ; assert_eq ! (ctxs [0] . userhash , true) ; assert_eq ! (ctxs [0] . algorithm , Algorithm :: Sha512Trunc256) ; assert_eq ! (ctxs [0] . qop . 0 , Qop :: Auth as u8) ; assert_eq ! (ctxs [0] . nc , 0) ; let params = crate :: PasswordParams { username : "J\u{E4}s\u{F8}n Doe" , password : "Secret, or not?" , uri : "/doe.json" , body : None , method : "GET" , } ; assert_eq ! (& mut ctxs [0] . respond_with_testing_cnonce (& params , "NTg6RKcb9boFIAS3KrFK9BGeh+iDa/sm6jUMp2wds69v") . unwrap () , "\
            Digest \
            username=\"793263caabb707a56211940d90411ea4a575adeccb7e360aeb624ed06ece9b0b\", \
            userhash=true, \
            realm=\"api@example.org\", \
            uri=\"/doe.json\", \
            nonce=\"5TsQWLVdgBdmrQ0XsxbDODV+57QdFR34I9HAbC/RVvkK\", \
            algorithm=SHA-512-256, \
            nc=00000001, \
            cnonce=\"NTg6RKcb9boFIAS3KrFK9BGeh+iDa/sm6jUMp2wds69v\", \
            qop=auth, \
            response=\"3798d4131c277846293534c3edc11bd8a5e4cdcbff78b05db9d95eeb1cec68a5\", \
            opaque=\"HRPCssKJSGjCrkzDg8OhwpzCiGPChXYjwrI2QmXDnsOS\"") ; assert_eq ! (ctxs [0] . nc , 1) ; ctxs [0] . userhash = false ; ctxs [0] . nc = 0 ; assert_eq ! (& mut ctxs [0] . respond_with_testing_cnonce (& params , "NTg6RKcb9boFIAS3KrFK9BGeh+iDa/sm6jUMp2wds69v") . unwrap () , "\
            Digest \
            username*=UTF-8''J%C3%A4s%C3%B8n%20Doe, \
            realm=\"api@example.org\", \
            uri=\"/doe.json\", \
            nonce=\"5TsQWLVdgBdmrQ0XsxbDODV+57QdFR34I9HAbC/RVvkK\", \
            algorithm=SHA-512-256, \
            nc=00000001, \
            cnonce=\"NTg6RKcb9boFIAS3KrFK9BGeh+iDa/sm6jUMp2wds69v\", \
            qop=auth, \
            response=\"3798d4131c277846293534c3edc11bd8a5e4cdcbff78b05db9d95eeb1cec68a5\", \
            opaque=\"HRPCssKJSGjCrkzDg8OhwpzCiGPChXYjwrI2QmXDnsOS\"") ; assert_eq ! (ctxs [0] . nc , 1) ; } # [test] fn rfc2069 () { let www_authenticate = "\
            Digest \
            realm=\"testrealm@host.com\", \
            nonce=\"dcd98b7102dd2f0e8b11d0f600bfb0c093\", \
            opaque=\"5ccc069c403ebaf9f0171e9517f40e41\"" ; let challenges = dbg ! (crate :: parse_challenges (www_authenticate) . unwrap ()) ; assert_eq ! (challenges . len () , 1) ; let ctxs : Result < Vec < _ > , _ > = challenges . iter () . map (DigestClient :: try_from) . collect () ; let mut ctxs = dbg ! (ctxs . unwrap ()) ; assert_eq ! (ctxs . len () , 1) ; assert_eq ! (ctxs [0] . qop . 0 , Qop :: Auth as u8) ; assert_eq ! (ctxs [0] . rfc2069_compat , true) ; let params = crate :: PasswordParams { username : "Mufasa" , password : "CircleOfLife" , uri : "/dir/index.html" , body : None , method : "GET" , } ; assert_eq ! (& mut ctxs [0] . respond_with_testing_cnonce (& params , "unused") . unwrap () , "\
            Digest \
            username=\"Mufasa\", \
            realm=\"testrealm@host.com\", \
            uri=\"/dir/index.html\", \
            nonce=\"dcd98b7102dd2f0e8b11d0f600bfb0c093\", \
            response=\"1949323746fe6a43ef61f9606e7febea\", \
            opaque=\"5ccc069c403ebaf9f0171e9517f40e41\"" ,) ; assert_eq ! (ctxs [0] . nc , 1) ; } # [test] fn size () { assert_eq ! (dbg ! (std :: mem :: size_of ::< DigestClient > ()) , dbg ! (std :: mem :: size_of ::< Option < DigestClient >> ()) ,) } }
};
}
