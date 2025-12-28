macro_rules! deps {
    () => {
        AsymmetricPublicKey!();
        AsymmetricSecretKey!();
    };
}

macro_rules! AsymmetricKeyPair {
    () => {
        deps!();
        # [derive (Debug , Clone)] # [doc = " A keypair of an [`AsymmetricSecretKey`] and its corresponding [`AsymmetricPublicKey`]."] pub struct AsymmetricKeyPair < V > { # [doc = " The [`AsymmetricSecretKey`]."] pub public : AsymmetricPublicKey < V > , # [doc = " The [`AsymmetricPublicKey`]."] pub secret : AsymmetricSecretKey < V > , }
    };
}

AsymmetricKeyPair!();