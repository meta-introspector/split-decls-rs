use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Probe the current system for the "cert file" and "cert dir" variables that
/// OpenSSL typically requires.
///
/// The probe result is returned as a [`ProbeResult`] structure here.
pub fn probe() -> ProbeResult {
    let mut result = probe_from_env();
    for certs_dir in candidate_cert_dirs() {
        let cert_filenames = [
            "cert.pem",
            "certs.pem",
            "ca-bundle.pem",
            "cacert.pem",
            "ca-certificates.crt",
            "certs/ca-certificates.crt",
            "certs/ca-root-nss.crt",
            "certs/ca-bundle.crt",
            "CARootCertificates.pem",
            "tls-ca-bundle.pem",
        ];
        if result.cert_file.is_none() {
            result.cert_file = cert_filenames
                .iter()
                .map(|fname| certs_dir.join(fname))
                .find(|p| p.exists());
        }
        if result.cert_dir.is_none() {
            let cert_dir = certs_dir.join("certs");
            if cert_dir.exists() {
                result.cert_dir = Some(cert_dir);
            }
        }
        if result.cert_file.is_some() && result.cert_dir.is_some() {
            break;
        }
    }
    result
}
