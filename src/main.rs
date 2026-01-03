use split_decls_genesis::rustc_matrix::RustcMatrix;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    RustcMatrix::run().await
}
