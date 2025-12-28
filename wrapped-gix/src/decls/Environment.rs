macro_rules! deps {
    () => {
        Permissions!();
        Note!();
        Clone!();
    };
}

macro_rules! Environment {
    () => {
        deps!();
        # [doc = " Permissions related to the usage of environment variables"] # [derive (Debug , Clone , Copy)] pub struct Environment { # [doc = " Control whether resources pointed to by `XDG_CONFIG_HOME` can be used when looking up common configuration values."] # [doc = ""] # [doc = " Note that [`gix_sec::Permission::Forbid`] will cause the operation to abort if a resource is set via the XDG config environment."] pub xdg_config_home : gix_sec :: Permission , # [doc = " Control the way resources pointed to by the home directory (similar to `xdg_config_home`) may be used."] pub home : gix_sec :: Permission , # [doc = " Control if environment variables to configure the HTTP transport, like `http_proxy` may be used."] # [doc = ""] # [doc = " Note that http-transport related environment variables prefixed with `GIT_` may also be included here"] # [doc = " if they match this category like `GIT_HTTP_USER_AGENT`."] pub http_transport : gix_sec :: Permission , # [doc = " Control if the `EMAIL` environment variables may be read."] # [doc = ""] # [doc = " Note that identity related environment variables prefixed with `GIT_` may also be included here"] # [doc = " if they match this category."] pub identity : gix_sec :: Permission , # [doc = " Control if environment variables related to the object database are handled. This includes features and performance"] # [doc = " options alike."] pub objects : gix_sec :: Permission , # [doc = " Control if resources pointed to by `GIT_*` prefixed environment variables can be used, **but only** if they"] # [doc = " are not contained in any other category. This is a catch-all section."] pub git_prefix : gix_sec :: Permission , # [doc = " Control if resources pointed to by `SSH_*` prefixed environment variables can be used (like `SSH_ASKPASS`)"] pub ssh_prefix : gix_sec :: Permission , }
    };
}

Environment!()