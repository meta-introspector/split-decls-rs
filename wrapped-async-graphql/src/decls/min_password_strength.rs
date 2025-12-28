macro_rules! min_password_strength {
    () => {
        # [cfg (feature = "password-strength-validator")] mod min_password_strength ;
    };
}

min_password_strength!()