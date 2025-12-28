macro_rules! macro_13 {
    () => {
        bitflags ! { # [doc = " Types of credentials that can be requested by a credential callback."] # [derive (Clone , Copy , Debug , Eq , PartialEq , PartialOrd , Ord , Hash)] pub struct CredentialType : u32 { # [allow (missing_docs)] const USER_PASS_PLAINTEXT = raw :: GIT_CREDTYPE_USERPASS_PLAINTEXT as u32 ; # [allow (missing_docs)] const SSH_KEY = raw :: GIT_CREDTYPE_SSH_KEY as u32 ; # [allow (missing_docs)] const SSH_MEMORY = raw :: GIT_CREDTYPE_SSH_MEMORY as u32 ; # [allow (missing_docs)] const SSH_CUSTOM = raw :: GIT_CREDTYPE_SSH_CUSTOM as u32 ; # [allow (missing_docs)] const DEFAULT = raw :: GIT_CREDTYPE_DEFAULT as u32 ; # [allow (missing_docs)] const SSH_INTERACTIVE = raw :: GIT_CREDTYPE_SSH_INTERACTIVE as u32 ; # [allow (missing_docs)] const USERNAME = raw :: GIT_CREDTYPE_USERNAME as u32 ; } }
    };
}

macro_13!()