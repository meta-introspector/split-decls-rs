macro_rules! SshHostKeyType {
    () => {
        # [doc = " The SSH host key type."] # [derive (Copy , Clone , Debug)] # [non_exhaustive] pub enum SshHostKeyType { # [doc = " Unknown key type"] Unknown = raw :: GIT_CERT_SSH_RAW_TYPE_UNKNOWN as isize , # [doc = " RSA key type"] Rsa = raw :: GIT_CERT_SSH_RAW_TYPE_RSA as isize , # [doc = " DSS key type"] Dss = raw :: GIT_CERT_SSH_RAW_TYPE_DSS as isize , # [doc = " ECDSA 256 key type"] Ecdsa256 = raw :: GIT_CERT_SSH_RAW_TYPE_KEY_ECDSA_256 as isize , # [doc = " ECDSA 384 key type"] Ecdsa384 = raw :: GIT_CERT_SSH_RAW_TYPE_KEY_ECDSA_384 as isize , # [doc = " ECDSA 521 key type"] Ecdsa521 = raw :: GIT_CERT_SSH_RAW_TYPE_KEY_ECDSA_521 as isize , # [doc = " ED25519 key type"] Ed255219 = raw :: GIT_CERT_SSH_RAW_TYPE_KEY_ED25519 as isize , }
    };
}

SshHostKeyType!()