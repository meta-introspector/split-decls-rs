macro_rules! candidate_cert_dirs {
    () => {
        # [doc = " Probe the system for the directory in which CA certificates should likely be"] # [doc = " found."] # [doc = ""] # [doc = " This will only search known system locations."] pub fn candidate_cert_dirs () -> impl Iterator < Item = & 'static Path > { ["/var/ssl" , "/usr/share/ssl" , "/usr/local/ssl" , "/usr/local/openssl" , "/usr/local/etc/openssl" , "/usr/local/share" , "/usr/lib/ssl" , "/usr/ssl" , "/etc/openssl" , "/etc/pki/ca-trust/extracted/pem" , "/etc/pki/tls" , "/etc/ssl" , "/etc/certs" , "/opt/etc/ssl" , # [cfg (target_os = "android")] "/data/data/com.termux/files/usr/etc/tls" , # [cfg (target_os = "haiku")] "/boot/system/data/ssl" ,] . iter () . map (Path :: new) . filter (| p | p . exists ()) }
    };
}

candidate_cert_dirs!()