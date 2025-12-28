macro_rules! deps {
    () => {
        Error!();
        RemoteCreate!();
        CheckoutBuilder!();
        RemoteCallbacks!();
        FetchOptions!();
        CloneLocal!();
    };
}

macro_rules! RepoBuilder {
    () => {
        deps!();
        # [doc = " A builder struct which is used to build configuration for cloning a new git"] # [doc = " repository."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " Cloning using SSH:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use git2::{Cred, Error, RemoteCallbacks};"] # [doc = " use std::env;"] # [doc = " use std::path::Path;"] # [doc = ""] # [doc = "   // Prepare callbacks."] # [doc = "   let mut callbacks = RemoteCallbacks::new();"] # [doc = "   callbacks.credentials(|_url, username_from_url, _allowed_types| {"] # [doc = "     Cred::ssh_key("] # [doc = "       username_from_url.unwrap(),"] # [doc = "       None,"] # [doc = "       Path::new(&format!(\"{}/.ssh/id_rsa\", env::var(\"HOME\").unwrap())),"] # [doc = "       None,"] # [doc = "     )"] # [doc = "   });"] # [doc = ""] # [doc = "   // Prepare fetch options."] # [doc = "   let mut fo = git2::FetchOptions::new();"] # [doc = "   fo.remote_callbacks(callbacks);"] # [doc = ""] # [doc = "   // Prepare builder."] # [doc = "   let mut builder = git2::build::RepoBuilder::new();"] # [doc = "   builder.fetch_options(fo);"] # [doc = ""] # [doc = "   // Clone the project."] # [doc = "   builder.clone("] # [doc = "     \"git@github.com:rust-lang/git2-rs.git\","] # [doc = "     Path::new(\"/tmp/git2-rs\"),"] # [doc = "   );"] # [doc = " ```"] pub struct RepoBuilder < 'cb > { bare : bool , branch : Option < CString > , local : bool , hardlinks : bool , checkout : Option < CheckoutBuilder < 'cb > > , fetch_opts : Option < FetchOptions < 'cb > > , clone_local : Option < CloneLocal > , remote_create : Option < Box < RemoteCreate < 'cb > > > , }
    };
}

RepoBuilder!()