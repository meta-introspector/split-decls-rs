macro_rules! deps {
    () => {
        Options!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl Default for Options { fn default () -> Self { Options { extra_headers : vec ! [] , follow_redirects : Default :: default () , low_speed_limit_bytes_per_second : 0 , low_speed_time_seconds : 0 , proxy : None , no_proxy : None , proxy_auth_method : Default :: default () , proxy_authenticate : None , user_agent : None , connect_timeout : None , verbose : false , ssl_ca_info : None , ssl_version : None , ssl_verify : true , http_version : None , backend : None , } } }
    };
}

impl_89!()