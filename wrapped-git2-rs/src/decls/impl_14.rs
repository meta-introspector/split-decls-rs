macro_rules! impl_14 {
    () => {
        impl CredentialType { is_bit_set ! (is_user_pass_plaintext , CredentialType :: USER_PASS_PLAINTEXT) ; is_bit_set ! (is_ssh_key , CredentialType :: SSH_KEY) ; is_bit_set ! (is_ssh_memory , CredentialType :: SSH_MEMORY) ; is_bit_set ! (is_ssh_custom , CredentialType :: SSH_CUSTOM) ; is_bit_set ! (is_default , CredentialType :: DEFAULT) ; is_bit_set ! (is_ssh_interactive , CredentialType :: SSH_INTERACTIVE) ; is_bit_set ! (is_username , CredentialType :: USERNAME) ; }
    };
}

impl_14!()