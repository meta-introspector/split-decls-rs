use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Prompt the user for a token.
pub fn read_token(
    login_options: &LoginOptions<'_>,
    registry: &RegistryInfo<'_>,
) -> Result<Secret<String>, Error> {
    if let Some(token) = &login_options.token {
        return Ok(token.to_owned());
    }
    if let Some(url) = login_options.login_url {
        eprintln!("please paste the token found on {url} below");
    } else if let Some(name) = registry.name {
        eprintln!("please paste the token for {name} below");
    } else {
        eprintln!("please paste the token for {} below", registry.index_url);
    }
    Ok(Secret::from(read_line().map_err(Box::new)?))
}
