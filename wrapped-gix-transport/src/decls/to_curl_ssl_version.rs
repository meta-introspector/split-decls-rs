macro_rules! to_curl_ssl_version {
    () => {
        fn to_curl_ssl_version (vers : SslVersion) -> curl :: easy :: SslVersion { use curl :: easy :: SslVersion :: * ; match vers { SslVersion :: Default => Default , SslVersion :: TlsV1 => Tlsv1 , SslVersion :: SslV2 => Sslv2 , SslVersion :: SslV3 => Sslv3 , SslVersion :: TlsV1_0 => Tlsv10 , SslVersion :: TlsV1_1 => Tlsv11 , SslVersion :: TlsV1_2 => Tlsv12 , SslVersion :: TlsV1_3 => Tlsv13 , } }
    };
}

to_curl_ssl_version!();